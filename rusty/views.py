from django.shortcuts import render,redirect


def home_page(request):
    return render(request, 'index.html')
